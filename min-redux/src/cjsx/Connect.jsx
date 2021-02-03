import React from 'react'
import PropType from 'prop-types'

// https://juejin.cn/post/6923922875191656462#heading-9

const connect = (
    mapStateToProps = () => ({}),
    mapDispatchToProps = () => ({})
) => Component => {
    class Connected extends React.Component {

        onStoreOrPropsChange(props) {
            const store = this.context
            const state = store.getState();
            const stateProps = mapStateToProps(state, props)
            const dispatchProps = mapDispatchToProps(store.dispatch, props)
            this.setState({
                ...stateProps,
                ...dispatchProps
            })
        }

        componentWillMount() {
            const { store } = this.context;
            this.onStoreOrPropsChange(this.props);
            this.unsubscribe = store.subscribe(() => this.onStoreOrPropsChange(this.props));
        }

        componentWillReceiveProps(nextProps) {
            this.onStoreOrPropsChange(nextProps);
        }

        componentWillUnmount() {
            this.unsubscribe();
        }

        render() {
            return <Component {...this.props} {...this.state} />;
        }

    }

    Connected.contextType = {
        store: PropType.object
    }


    return Connected
}

export default connect