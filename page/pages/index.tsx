import Head from 'next/head'
import { Inter } from 'next/font/google'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  return (
    <>
      <Head>
        <title>GrepTodo</title>
        <meta name="description" content="Grep todo history from repository" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <section className="main-template-areas">

        <header className="header-template-areas border-double border-5 border-rd-b-5 b-t-0 min-h-lg">
          <div className="logo h-30 w-30 c-yellow i-svg-spinners-pulse-rings-multiple"></div>
          <button className="search-bar grid-self-center bg-yellow vertical-mid btn w-10rem">Search</button>
          <title className="flex bg-red h-4rem">
            GrepTodo
          </title>
        </header>

        <main className="bg-light-blue">
        </main >

        <footer className="bg-green c-white px">
          footer
        </footer>

      </section>
    </>
  )
}
