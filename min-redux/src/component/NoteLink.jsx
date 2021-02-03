import React from 'react'

import NoteTitle from './NoteTitle.jsx'

const NoteLink = ({ note, onOpenNote }) => {
    return (
        <li className="note-list-item">
            <a href="#" onClick={() => onOpenNote(note.id)}>
                <NoteTitle note={note} />
            </a>
        </li>
    )
}

export default NoteLink