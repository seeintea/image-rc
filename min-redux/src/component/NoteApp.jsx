import React from 'react'
import NoteEditor from './NoteEditor.jsx'
import NoteList from './NoteList.jsx'

const NoteApp = ({
    notes, openNoteId, onAddNote, onChangeNote, onOpenNote, onCloseNote
}) => (
    <div>
        {
            openNoteId ?
                <NoteEditor
                    note={notes[openNoteId]} onChangeNote={onChangeNote}
                    onCloseNote={onCloseNote}
                /> :
                <div>
                    <NoteList notes={notes} onOpenNote={onOpenNote} />
                    <button className="editor-button" onClick={onAddNote}>New Note</button>
                </div>
        }
    </div>
)

export default NoteApp