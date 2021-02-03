import React from 'react'

const NoteTitle = ({ note }) => {
    const title = note.content.split('\n')[0].replace(/^\s+|\s+$/g, '');
    if (title === '') {
        return <i>Untitled</i>
    }
    return <span>{title}</span>
}

export default NoteTitle