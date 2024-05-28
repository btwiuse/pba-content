import { Accordion } from './components/Accordion'

import './App.css'
import someJson from "./syllabus.json"
import Logo from "./images/powered-by-gear.png";

export const replaceText = (v: string) => v.replace(/\d+?(?=-)/g, "").replace(/-|_/g, " ")

export const App = () => {
  return (
    <>
      <header className="site-header site-header--small">
        <menu className="menu menu--header">
          <li className="menu-item logo">
            <img src={Logo} style={{ maxWidth: "70%" }} />
          </li>
          <li className="menu-item">
            <h2>Table of Contents</h2>
          </li>
        </menu>
      </header>
      <div className="accordion">
        {Object.entries(someJson).map(r => <Accordion title={replaceText(r[0])} content={r[1]} path={"syllabus/" + r[0]}/>)}
      </div>
    </>
  )
}
