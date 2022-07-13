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
  const [state, setState] = useState<{ fields: [string], game_state: string } | null>(null)
  const [updating, setUpdating] = useState(true)

  const { status } = useWallet()

  const connectedWallet = useConnectedWallet()

  useEffect(() => {
    const prefetch = async () => {
      if (connectedWallet) {
        const state: any = await query.getState(connectedWallet)
        console.log(state)
        setState(state)
      }
      setUpdating(false)
    }
    prefetch()
  }, [connectedWallet])

  const onClickPlay = async (field_num: number) => {
    if (connectedWallet) {
      setUpdating(true)
      await execute.play(connectedWallet, field_num)
      const state: any = await query.getState(connectedWallet)
      setState(state)
      setUpdating(false)
    }
  }

  const onClickReset = async () => {
    if (connectedWallet) {
      setUpdating(true)
      await execute.reset(connectedWallet)
      const state: any = await query.getState(connectedWallet)
      console.log(state)
      setState(state)
      setUpdating(false)
    }
  }

  return (
    <div className="App">
      <header className="App-header">
        <div style={{ display: 'inline' }}>
          {
            state ? (<table><tbody>{[0, 1, 2].map((row) =>
            (<tr key={row}>{[0, 1, 2].map((column) => {
              const i = 3 * row + column;
              return (<td key={i} onClick={async () => { await onClickPlay(i) }}>
                {state.fields[i] === 'Empty' ? '/' : state.fields[i]}
              </td>)
            })}</tr>)
            )}</tbody></table>) : ''
          }
        </div>
        {state ? state.game_state : ''}
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
