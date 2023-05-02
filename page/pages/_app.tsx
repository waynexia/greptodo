import type { AppProps } from 'next/app'
import 'uno.css'
import '@/styles/grid-layout.css'
import '@/styles/input-area.css'
import '@unocss/reset/tailwind.css'

export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />
}
