const CREATE_NOTE = "CREATE_NOTE"
const UPDATE_NOTE = "UPDATE_NOTE"
const OPEN_NOTE = 'OPEN_NOTE';
const CLOSE_NOTE = 'CLOSE_NOTE';

const initialState = {
    nextNoteId: 1,
    notes: {},
    openNoteId: null
}

const reducer = (state = initialState, action) => {
    switch (action.type) {
        case CREATE_NOTE: {
            const id = state.nextNoteId
            const newNote = {
                id,
                content: ''
            }
            return {
                ...state,
                nextNoteId: id + 1,
                openNoteId: id,
                notes: {
                    ...state.notes,
                    [id]: newNote,
                }
            }
        }
        case UPDATE_NOTE: {
            const { id, content } = action
            const editedNote = {
                ...state.notes[id],
                content
            }
            return {
                ...state,
                notes: {
                    ...state.notes,
                    [id]: editedNote
                }
            }
        }
        case OPEN_NOTE: {
            return {
                ...state,
                openNoteId: action.id
            };
        }
        case CLOSE_NOTE: {
            return {
                ...state,
                openNoteId: null
            };
        }
        default:
            return state
    }
}


module.exports = {
    CREATE_NOTE,
    UPDATE_NOTE,
    OPEN_NOTE,
    CLOSE_NOTE,
    reducer
}