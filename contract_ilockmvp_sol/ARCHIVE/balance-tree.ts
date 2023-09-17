import MerkleTree from './merkle-tree'
import { BigNumber, utils } from 'ethers'

export default class BalanceTree {
	private readonly tree: MerkleTree
	constructor(balances: { account: string; share: BigNumber; owes: BigNumber; pool: BigNumber }[]) {
		this.tree = new MerkleTree(
		balances.map(({ account, share, owes, pool }, index) => {
        		return BalanceTree.toNode(index, account, share, owes, pool)
      		})
    	)}

	public static verifyProof(
		index: number | BigNumber,
		account: string,
		share: BigNumber,
		owes: BigNumber,
		pool: BigNumber,
		proof: Buffer[],
		root: Buffer
	): boolean {
		let pair = BalanceTree.toNode(index, account, share, owes, pool)
		for (const item of proof) {
			pair = MerkleTree.combinedHash(pair, item)
		}

		return pair.equals(root)
	}

		// keccak256(abi.encode(index, account, share))
	public static toNode(index: number | BigNumber, account: string, share: BigNumber, owes: BigNumber, pool: BigNumber): Buffer {
		return Buffer.from(
			utils.solidityKeccak256(
				['uint256', 'address', 'uint256', 'uint256', 'uint256'],
				[index, account, share, owes, pool]).substr(2),
			'hex'
		)
	}

	public getHexRoot(): string {
		return this.tree.getHexRoot()
	}

		// returns the hex bytes32 values of the proof
	public getProof(index: number | BigNumber, account: string, share: BigNumber, owes: BigNumber, pool: BigNumber): string[] {
		return this.tree.getHexProof(BalanceTree.toNode(index, account, share, owes, pool))
	}

}
