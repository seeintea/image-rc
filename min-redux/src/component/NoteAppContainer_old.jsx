import React from 'react'
import NoteApp from './NoteApp.jsx'
import { CREATE_NOTE, UPDATE_NOTE, OPEN_NOTE, CLOSE_NOTE } from '../core/reducer'

class NoteAppContainer extends React.Component {
    constructor(props) {
        super();
        this.state = props.store.getState();
        this.onAddNote = this.onAddNote.bind(this);
        this.onChangeNote = this.onChangeNote.bind(this);
        this.onOpenNote = this.onOpenNote.bind(this);
        this.onCloseNote = this.onCloseNote.bind(this);
    }
    componentWillMount() {
        this.unsubscribe = this.props.store.subscribe(() =>
            this.setState(this.props.store.getState())
        );
    }
    componentWillUnmount() {
        this.unsubscribe();
    }
    onAddNote() {
        this.props.store.dispatch({
            type: CREATE_NOTE
        });
    }
    onChangeNote(id, content) {
        this.props.store.dispatch({
            type: UPDATE_NOTE,
            id,
            content
        });
    }
    onOpenNote(id) {
        this.props.store.dispatch({
            type: OPEN_NOTE,
            id
        });
    }
    onCloseNote() {
        this.props.store.dispatch({
            type: CLOSE_NOTE
        });
    }
    render() {
        return (
            <NoteApp
                {...this.state}
                onAddNote={this.onAddNote}
                onChangeNote={this.onChangeNote}
                onOpenNote={this.onOpenNote}
                onCloseNote={this.onCloseNote}
            />
        );
    }
}

export default NoteAppContainer