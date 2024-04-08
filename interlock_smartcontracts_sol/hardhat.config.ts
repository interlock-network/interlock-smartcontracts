import '@openzeppelin/hardhat-upgrades'
import '@nomicfoundation/hardhat-chai-matchers'
import '@nomicfoundation/hardhat-ethers'
import '@nomicfoundation/hardhat-verify'
import '@typechain/hardhat'
import 'hardhat-abi-exporter'
import 'hardhat-gas-reporter'
import { HardhatUserConfig } from 'hardhat/config'

import * as dotenv from 'dotenv'
dotenv.config({ path: './.env.dev' })

const config: HardhatUserConfig = {
  abiExporter: {
    path: './abi',
    clear: true,
    flat: false,
    except: ['@openzeppelin'],
    spacing: 2,
    runOnCompile: true,
    pretty: false
  },
  gasReporter: {
    coinmarketcap: process.env.CMC_API_KEY,
    enabled: !!process.env.REPORT_GAS,
    showTimeSpent: true
  },
  defender: {
    apiKey: process.env.DEFENDER_API_KEY as string,
    apiSecret: process.env.DEFENDER_API_SECRET as string
  },
  networks: {
    hardhat: {},
    arbitrumSepolia: {
      url: 'https://sepolia-rollup.arbitrum.io/rpc',
      chainId: 421614
    }
  },
  etherscan: {
    apiKey: {
      arbitrumSepolia: process.env.ARBITRUM_APIKEY as string
    }
  },
  solidity: {
    compilers: [
      {
        version: '0.8.20',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200
          },
          metadata: {
            // do not include the metadata hash, since this is machine dependent
            // and we want all generated code to be deterministic
            // https://docs.soliditylang.org/en/v0.8.24/metadata.html
            bytecodeHash: 'none'
          }
        }
      }
    ]
  },
  paths: {
    sources: './contracts',
    tests: './test',
    cache: './cache',
    artifacts: './artifacts'
  }
}

export default config
