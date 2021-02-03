import React from 'react'

const NoteEditor = ({ note, onChangeNote, onCloseNote }) => {
    return (
        <>
            <div>
                <textarea
                    className='editor-content'
                    autoFocus
                    value={note.content}
                    onChange={event => onChangeNote(note.id, event.target.value)}
                    rows={10}
                    cols={80}
                >
                </textarea>
            </div>
            <button className="editor-button" onClick={onCloseNote}>提交</button>
        </>
    )
}

export default NoteEditor