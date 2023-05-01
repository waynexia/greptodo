import type { AppProps } from 'next/app'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import '@/styles/grid-layout.css'

export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />
}
