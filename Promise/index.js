// 定义三种状态
// 等待态
const PENDING = 'pending'
// 执行态
const FULFILLED = 'fulfilled'
// 拒绝态
const REJECTED = 'rejected'

// 工具函数
const isFunction = (fn) => {
    return typeof fn === 'function'
}

const resolvePromise = (promise, x, reslove, reject) => {
    // 2.3.1 If promise and x refer to the same object, reject promise with a TypeError as the reason.
    if(promise === x) {
        return reject(new TypeError('Chaining cycle detected for promise #<Promise>'))
    }
    // 2.3.3.3.3
    let called
    // 2.3.3 Otherwise, if x is an object or function
    if((typeof x === 'object' && x != null) || isFunction(x)) {
        try{
            // 2.3.3.1 Let then be x.then.
            let then = x.then 
            // 2.3.3.3 If then is a function, call it with x as this
            if(isFunction(then)) {
                then.call(x, (y) => {
                    if(called) return
                    called = true
                    // 2.3.3.3.1 If/when resolvePromise is called with a value y, run [[Resolve]](promise, y).
                    resolvePromise(promise, y, reslove, reject)
                }, (r) => {
                    if(called) return
                    called = true
                    // 2.3.3.3.2 If/when rejectPromise is called with a reason r, reject promise with r.
                    reject(r)
                })
            } else {
                // 2.3.3.4 If x is not an object or function, fulfill promise with x.
                reslove(x)
            }
        }catch(e) {
            //2.3.3.2 If retrieving the property x.then results in a thrown exception e, reject promise with e as the reason.
            if(called) return
            called = true
            reject(e)
        }        
    } else {
        // 2.3.4 If x is not an object or function, fulfill promise with x.
        reslove(x)
    }
}


class Promise {

    // 构造函数
    constructor(executer) {

        this.status = PENDING
        // 终值
        this.value = undefined
        // 拒绝原因
        this.reason = undefined

        // 回调函数
        this.onFulfilledCallbacks = []
        this.onRejectedCallbacks = []

        const resolve = (value) => {
            if(this.status === PENDING) {
                this.status = FULFILLED
                this.value = value
                this.onFulfilledCallbacks.forEach(fn => fn())
            }
        }

        const reject = (reason) => {
            if(this.status === PENDING) {
                this.status = REJECTED
                this.reason = reason
                this.onRejectedCallbacks.forEach(fn => fn())
            }
        }

        try {
            executer(resolve, reject)
        } catch(e) {
            reject(e)
        }

    }

    then(onFulfilled, onRejected) {
        // 2.2.1 Both onFulfilled and onRejected are optional arguments
        onFulfilled = isFunction(onFulfilled) ? onFulfilled : (val) => val
        onRejected = isFunction(onRejected) ? onRejected : (err) => { throw err }
        
        // 2.2.7 then must return a promise
        let p = new Promise((reslove, reject) => {
            // 2.2.2 
            if(this.status === FULFILLED) {
                setTimeout(() => {
                    try{
                        let val = onFulfilled(this.value)
                        resolvePromise(p, val, reslove, reject)
                    }catch(e) {
                        reject(e)
                    }
                }, 0)  
            }

            // 2.2.3
            if(this.status === REJECTED) {
                setTimeout(() => {
                    try{
                        let err = onRejected(this.reason)
                        resolvePromise(p, err, reslove, reject)
                    }catch(e) {
                        reject(e)
                    }
                }, 0)  
            }

            if(this.status === PENDING) {
                this.onFulfilledCallbacks.push(() => {
                    setTimeout(() => {
                        try{
                            let val = onFulfilled(this.value)
                            resolvePromise(p, val, reslove, reject)
                        }catch(e) {
                            reject(e)
                        }
                    }, 0)  
                })
                this.onRejectedCallbacks.push(() => {
                    setTimeout(() => {
                        try{
                            let err = onRejected(this.reason)
                            resolvePromise(p, err, reslove, reject)
                        }catch(e) {
                            reject(e)
                        }
                    }, 0)  
                })
            }    
        })

        return p
    }


    catch(err) {
        return this.then(null, err)
    }

    finally(cb) {
        return this.then(
            (data) => {
                return Promise.resolve(cb()).then(() => data)
            },
            (err) => {
                return Promise.resolve(cb()).then(() => {
                    throw err
                })
            }
        )
    }
}

// 测试
Promise.defer = Promise.deferred = () => {
    let dfd = []
    dfd.promise = new Promise((resolve,reject)=>{
        dfd.resolve = resolve
        dfd.reject = reject
    })
    return dfd
}

module.exports = Promise