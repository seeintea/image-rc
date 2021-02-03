import React from 'react'
import NoteLink from './NoteLink.jsx'

const NoteList = ({ notes, onOpenNote }) => {
    return (
        <ul className="note-list">
            {
                Object.keys(notes).map(id => {
                    return (
                        <NoteLink
                            key={id}
                            note={notes[id]}
                            onOpenNote={onOpenNote}
                        />
                    )
                })
            }
        </ul>
    )
}

export default NoteList
