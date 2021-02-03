/**
 * 1. 创建一个空的简单 JavaScript 对象（即{}）；
 * 2. 链接该对象（即设置该对象的构造函数）到另一个对象 ；
 * 3. 将步骤 1 新创建的对象作为this的上下文 ；
 * 4. 如果该函数没有返回对象，则返回this。
 */


function _new(fn, ...args){
    // 1. 创建一个空的简单 JavaScript 对象（即{}）
    let ob = Object.create(null)
        
    // 2. 链接该对象（即设置该对象的构造函数）到另一个对象
    const constructor = fn.prototype
    ob.__proto__ = constructor

    // 3. 将步骤 1 新创建的对象作为this的上下文
    const res = fn.apply(ob, args)

    return typeof res === 'object' ? res : ob   
}

module.exports = _new
