import './App.css'
import { useEffect, useState } from 'react'
import {
  useWallet,
  useConnectedWallet,
  WalletStatus,
} from '@terra-money/wallet-provider'

import * as execute from './contract/execute'
import * as query from './contract/query'
import { ConnectWallet } from './components/ConnectWallet'

const App = () => {
  const [fields, setFields] = useState(null)
  const [updating, setUpdating] = useState(true)

  const { status } = useWallet()

  const connectedWallet = useConnectedWallet()

  useEffect(() => {
    const prefetch = async () => {
      if (connectedWallet) {
        const { fields }: any = await query.getFields(connectedWallet)
        setFields(fields)
      }
      setUpdating(false)
    }
    prefetch()
  }, [connectedWallet])

  const onClickPlay = async (field_num: number) => {
    if (connectedWallet) {
      setUpdating(true)
      await execute.play(connectedWallet, field_num)
      const { fields }: any = await query.getFields(connectedWallet)
      setFields(fields)
      setUpdating(false)
    }
  }

  const onClickReset = async () => {
    if (connectedWallet) {
      setUpdating(true)
      await execute.reset(connectedWallet)
      const { fields }: any = await query.getFields(connectedWallet)
      setFields(fields)
      setUpdating(false)
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: 'inline' }}>
          {
            fields ? (<table>{[0, 1, 2].map((row) =>
            (<tr>{[0, 1, 2].map((column) => {
              const i = 3 * row + column;
              return (<td key={i} onClick={async () => { await onClickPlay(i) }}>
                {fields[i] === 'Empty' ? '/' : fields[i]}
              </td>)
            })}</tr>)
            )}</table>) : ''
          }
        </div>
        {updating ? '(updating . . .)' : ''}
        {status === WalletStatus.WALLET_CONNECTED && (
          <div style={{ display: 'inline' }}>
            <button onClick={onClickReset} type="button">
              {' '}
              reset{' '}
            </button>
          </div>
        )}
      </header>
      <ConnectWallet />
    </div>
  )
}

export default App
