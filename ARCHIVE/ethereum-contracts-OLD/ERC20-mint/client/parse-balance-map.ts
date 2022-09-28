import { BigNumber, utils } from 'ethers'
import BalanceTree from './balance-tree'

const { isAddress, getAddress } = utils

// This is the blob that gets distributed and pinned to IPFS.
// It is completely sufficient for recreating the entire merkle tree.
// Anyone can verify that all air drops are included in the tree,
// and the tree has no additional distributions.
interface MerkleDistributorInfo {
  merkleRoot: string
  tokenTotal: string
  claims: {
    [account: string]: {
      index: number
      share: string
      owes: string
      pool: string
      proof: string[]
      flags?: {
        [flag: string]: boolean
      }
    }
  }
}

type OldFormat = { [account: string]: [number | string, number, number] }
type NewFormat = { address: string; earnings: string; oweswhat: string; whichpool: string; reasons: string }

export function parseBalanceMap(balances: OldFormat | NewFormat[]): MerkleDistributorInfo {
  // if balances are in an old format, process them
  const balancesInNewFormat: NewFormat[] = Array.isArray(balances)
    ? balances
    : Object.keys(balances).map(
        (account): NewFormat => ({
          address: account,
          earnings: `0x${balances[account][0].toString(16)}`,
          oweswhat: `0x${balances[account][1].toString(16)}`,
          whichpool: `0x${balances[account][2].toString(16)}`,
          reasons: '',
        })
      )

  const dataByAddress = balancesInNewFormat.reduce<{
    [address: string]: { share: BigNumber; owes: BigNumber; pool: BigNumber; flags?: { [flag: string]: boolean } }
  }>((memo, { address: account, earnings, oweswhat, whichpool, reasons }) => {

    if (!isAddress(account)) {
      throw new Error(`Found invalid address: ${account}`)
    }
    const parsed = getAddress(account)
    if (memo[parsed]) throw new Error(`Duplicate address: ${parsed}`)
    const parsedNum1 = BigNumber.from(earnings)
    if (parsedNum1.lte(0)) throw new Error(`Invalid share for account: ${account}`)
    const parsedNum2 = BigNumber.from(oweswhat)
    //if (parsedNum2.lte(0)) throw new Error(`Invalid owes for account: ${account}`)
    const parsedNum3 = BigNumber.from(whichpool)
    //if (parsedNum3.lte(0)) throw new Error(`Invalid pool for account: ${account}`)


    const flags = {
      isSOCKS: reasons.includes('socks'),
      isLP: reasons.includes('lp'),
      isUser: reasons.includes('user'),
    }

    memo[parsed] = { share: parsedNum1, owes: parsedNum2, pool: parsedNum3, ...(reasons === '' ? {} : { flags }) }
    return memo
  }, {})


  const sortedAddresses = Object.keys(dataByAddress).sort()

  // construct a tree
  const tree = new BalanceTree(
    sortedAddresses.map((address) => ({ account: address, share: dataByAddress[address].share, owes: dataByAddress[address].owes, pool: dataByAddress[address].pool }))
  )

  // generate claims
  const claims = sortedAddresses.reduce<{
    [address: string]: { share: string; owes: string; pool: string; index: number; proof: string[]; flags?: { [flag: string]: boolean } }
  }>((memo, address, index) => {
    const { share, owes, pool, flags } = dataByAddress[address]
    memo[address] = {
      index,
      share: share.toHexString(),
      owes: owes.toHexString(),
      pool: pool.toHexString(),
      proof: tree.getProof(index, address, share, owes, pool),
      ...(flags ? { flags } : {}),
    }
    return memo
  }, {})

  const tokenTotal: BigNumber = sortedAddresses.reduce<BigNumber>(
    (memo, key) => memo.add(dataByAddress[key].share),
    BigNumber.from(0)
  )

  return {
    merkleRoot: tree.getHexRoot(),
    tokenTotal: tokenTotal.toHexString(),
    claims,
  }
}
