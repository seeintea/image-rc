import connect from '../cjsx/Connect.jsx'
import NoteApp from './NoteApp.jsx'
import { CREATE_NOTE, UPDATE_NOTE, OPEN_NOTE, CLOSE_NOTE } from '../core/reducer'

const mapStateToProps = state => ({
    notes: state.notes,
    openNoteId: state.openNoteId
});

const mapDispatchToProps = dispatch => ({
    onAddNote: () => dispatch({
        type: CREATE_NOTE
    }),
    onChangeNote: (id, content) => dispatch({
        type: UPDATE_NOTE,
        id,
        content
    }),
    onOpenNote: id => dispatch({
        type: OPEN_NOTE,
        id
    }),
    onCloseNote: () => dispatch({
        type: CLOSE_NOTE
    })
});

const NoteAppContainer = connect(
    mapStateToProps,
    mapDispatchToProps
)(NoteApp);


export default NoteAppContainer