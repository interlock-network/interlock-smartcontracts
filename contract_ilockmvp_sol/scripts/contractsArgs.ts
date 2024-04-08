type InterlockNetworkArgs = {
  params: {
    initialOwner: `0x${string}`
  }
  proxyAdminOwner?: `0x${string}`
}

const contractsArgs: Record<
  string,
  {
    InterlockNetwork: InterlockNetworkArgs
  }
> = {
  arbitrumSepolia: {
    InterlockNetwork: {
      params: {
        initialOwner: '0xf0Da5D820b6d0B9383Bb8e3ED7D3144b32B1349F'
      },
      proxyAdminOwner: '0xf0Da5D820b6d0B9383Bb8e3ED7D3144b32B1349F'
    }
  }
}

export default contractsArgs
