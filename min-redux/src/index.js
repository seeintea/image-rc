import React from 'react'
import ReactDOM, { render } from 'react-dom'

import { CREATE_NOTE, reducer } from './core/reducer'
import createStore from './core/action'

import NoteAppContainer from './component/NoteAppContainer.jsx'

const store = createStore(reducer)

store.dispatch({
    type: CREATE_NOTE
})

const renderApp = () => {
    ReactDOM.render(
        <NoteAppContainer store={store} />,
        document.getElementById("root")
    )
}

renderApp()