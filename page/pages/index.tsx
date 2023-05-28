import Head from 'next/head'
import Image from 'next/image'
import React from 'react'
import { useRef } from 'react'
import Background from "./background"
import axios from 'axios'
import OperationCount from './charts/operation_count'
import { FEED_SERVER_URL } from './consts'
import OperationHistory from './charts/operation_history'
import AuthorRank from './charts/author_rank'

function valid_search_repo(repo: string): boolean {
  if (repo.split('/').length !== 2) {
    // todo: add prompt to the input box
    console.log("invalid repo name, should be user/repo")
    return false
  } else {
    return true
  }
}

export default function Home() {
  const search_bar_ref = useRef<HTMLInputElement>(null)
  const [search_status, set_search_status] = React.useState<'idle' | 'searching' | 'done'>('idle')

  // Assume the repo string is valid
  function do_search(repo: string): void {
    console.log("start searching " + repo)

    var [user, repo_name] = repo.split('/')
    axios.post(`${FEED_SERVER_URL}/api/update_repo?org=` + user + '&repo=' + repo_name,
      {},
      { headers: { "Content-Type": "application/x-www-form-urlencoded" } }
    ).then(function (response) {
      console.log(response.data)
    });

    set_search_status('done')
  }

  const start_search = () => {
    if (search_bar_ref.current === null) {
      return
    }
    const repo = search_bar_ref.current.value
    if (!valid_search_repo(repo)) {
      return
    }
    set_search_status('searching')
    do_search(repo)
  }

  const reset_search = () => {
    set_search_status('idle')
  }

  const lucky_search = () => {
    set_search_status('searching')
    do_search('waynexia/unkai')
  }

  return (
    <>
      <Head>
        <title>GrepTodo</title>
        <meta name="description" content="Grep todo history from repository" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      {/* background */}
      <div className="no-scrollbar absolute z--10 min-h-screen min-w-screen overflow-clip">
        <Background user="waynexia" repo="unkai" />
      </div>

      {/* foreground */}
      <section className="main-template-areas">

        <header className="header-template-areas border-double border-5 border-rd-b-5 border-purple-2 b-t-0 min-h-lg backdrop-blur-lg" style={{ marginTop: search_status === "idle" ? 0 : "calc(4rem - 60vh)" }}>
          <Image className='logo grid-self-center' src="/greptodo.svg" alt="greptodo logo" width={180} height={180} />
          <div className="search-bar">
            <input className="grid-self-start vertical-mid w-full h-12 pa-4 search-input" placeholder="user/repo" ref={search_bar_ref} />
            <div className="flex flex-items-center flex-justify-around m-t-6">
              <button className="btn w-10rem flex flex-items-center flex-justify-center bg-purple-1 pa-2 border-rd-5" onClick={start_search}>
                <div className="h-6 w-6 i-mdi-magnify"></div>
                Search
              </button>
              <button className="btn w-10rem flex flex-items-center flex-justify-center bg-purple-1 pa-2 border-rd-5" onClick={lucky_search}>
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
            {/* todo: set font to Noto Serief */}
            <div className="text-10 font-extralight select-none">GrepTodo</div>
            {
              search_status === 'idle' ? <div /> :
                <button className="flex flex-items-center ma-4 c-red-5" onClick={reset_search}>
                  <div className="h-6 w-6 i-mdi-progress-close transition-500"></div>
                  Reset
                </button>
            }
          </div>
        </header>

        <main className="border-light-blue border-10 overflow-scroll no-scrollbar backdrop-blur-lg">
          <div className="flex flex-col flex-items-center">
            {
              {
                'idle': <div />,
                'searching': <div className="h-30 w-30 c-purple-7 i-svg-spinners-pulse-rings-multiple"></div>,
                'done': <div>search done</div>
              }[search_status]
            }
            <div className="">{search_status}</div>
            <div className="h-auto w-80%">
              <OperationCount repo_name="greptimeteam-greptimedb"></OperationCount>
              <OperationHistory repo_name="greptimeteam-greptimedb"></OperationHistory>
              <AuthorRank repo_name="greptimeteam-greptimedb"></AuthorRank>
            </div>
          </div>
        </main >

        <footer className="bg-emerald-6 c-white px flex">
          <div className="inline-flex flex-items-center m-r-4">
            <div className="i-mdi-source-commit"></div>
            <a href="https://github.com/waynexia/greptodo">waynexia/greptodo</a>
          </div>
          <div className="inline-flex flex-items-center m-r-4">
            <div className="i-mdi-source-branch"></div>
            <a href="https://github.com/waynexia/greptodo">main</a>
          </div>
          <div className="inline-flex flex-items-center m-r-4">
            <div className="i-mdi-cloud-check-outline"></div>
            <a href="https://greptime.cloud">Greptime Cloud</a>
          </div>

          <div className="inline-flex flex-items-center m-r-4">
            <div className="i-mdi-alpha-x-circle-outline m-r-1"></div>
            0
            <div className="i-mdi-alert-circle-outline m-l-1 m-r-1"></div>
            0
          </div>

        </footer>

      </section >
    </>
  )
}
