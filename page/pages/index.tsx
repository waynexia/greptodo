import Head from 'next/head'
import { Inter } from 'next/font/google'
import Image from 'next/image'
import React from 'react'
import ReactDOM from 'react-dom'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {

  const [search_status, set_search_status] = React.useState<'idle' | 'searching' | 'done'>('idle')

  const start_search = () => {
    set_search_status('searching')
  }

  const reset_search = () => {
    set_search_status('idle')
  }

  return (
    <>
      <Head>
        <title>GrepTodo</title>
        <meta name="description" content="Grep todo history from repository" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <section className="main-template-areas">

        <header className="header-template-areas border-double border-5 border-rd-b-5 border-purple-2 b-t-0 min-h-lg" style={{ marginTop: search_status === "idle" ? 0 : "calc(4rem - 60vh)" }}>
          <Image className='logo grid-self-center' src="/greptodo.svg" alt="greptodo logo" width={180} height={180} />
          <div className="search-bar">
            <input className="grid-self-start vertical-mid w-full h-12 pa-4 search-input" placeholder="user/repo" />
            <div className="flex flex-items-center flex-justify-around m-t-6">
              <button className="btn w-10rem flex flex-items-center flex-justify-center" onClick={start_search}>
                <div className="h-6 w-6 i-mdi-magnify"></div>
                Search
              </button>
              <button className="btn w-10rem flex flex-items-center flex-justify-center" onClick={start_search}>
                <div className="h-6 w-6 i-mdi-script-text-outline"></div>
                Feel Lucky
              </button>
            </div>
          </div>
          <div className="title-tab flex flex-justify-between flex-items-center h-4rem ">
            {
              search_status === 'idle' ? <div /> :
                <Image src="/greptodo.svg" alt="greptodo logo" width={50} height={50} className="ma-4 transition-500" />
            }
            <div className="text-10 font-extralight">GrepTodo</div>
            {
              search_status === 'idle' ? <div /> :
                <button className="flex flex-items-center ma-4 c-red-5" onClick={reset_search}>
                  <div className="h-6 w-6 i-mdi-progress-close transition-500"></div>
                  Reset
                </button>
            }
          </div>
        </header>

        {/* todo: this doesn't work */}
        <main className="border-light-blue border-10">
          {
            {
              'idle': <div />,
              'searching': <div className="h-30 w-30 c-white i-svg-spinners-pulse-rings-multiple"></div>,
              'done': <div>search done</div>
            }[search_status]
          }
        </main >

        <footer className="bg-green c-white px">
          footer
        </footer>

      </section >
    </>
  )
}
