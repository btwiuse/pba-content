---
title: 钱包交互
description: 钱包交互
duration: 30 min
---

# 钱包交互

---

## @polkadot/extension-dapp

- `web3Enable(dappName: string): Promise<InjectedExtension[]>` - to be called before anything else, retrieves the list of all injected extensions/providers
- `web3Accounts(): Promise<InjectedAccountWithMeta[]>` - returns a list of all the injected accounts, across all extensions (source in meta)
- `web3FromAddress(address: string): Promise<InjectedExtension>` - Retrieves a provider for a specific address
- `web3FromSource(name: string): Promise<InjectedExtension>` - Retrieves a provider identified by the name
- `isWeb3Injected: boolean` - boolean to indicate if `injectedWeb3` was found on the page

<br/>

文档: https://polkadot.js.org/docs/extension/

---

## 快速上手

```javascript
import { web3Accounts, web3Enable, web3FromAddress } from '@polkadot/extension-dapp';

// returns an array of all the injected sources
// (this needs to be called first, before other requests)
const allInjected = await web3Enable('my cool dapp');

// returns an array of { address, meta: { name, source } }
// meta.source contains the name of the extension that provides this account
const allAccounts = await web3Accounts();

// the address we use to use for signing, as injected
const SENDER = '5DTestUPts3kjeXSTMyerHihn1uwMfLj8vU8sqF7qYrFabHE';

// finds an injector for an address
const injector = await web3FromAddress(SENDER);

// sign and send our transaction - notice here that the address of the account
// (as retrieved injected) is passed through as the param to the `signAndSend`,
// the API then calls the extension to present to the user and get it signed.
// Once complete, the api sends the tx + signature via the normal process
api.tx.balances
  .transfer('5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ', 123456)
  .signAndSend(SENDER, { signer: injector.signer }, (status) => { ... });
```

---

## @gear-js/react-hooks

- useApi
- useAccount
- ...

NPM: https://www.npmjs.com/package/@gear-js/react-hooks