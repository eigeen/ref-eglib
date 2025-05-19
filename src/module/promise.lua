local function try(block)
    -- local try_fn = block[1]
    local try_fn = block[1]
    assert(type(try_fn) == "function", "try block must start with a function")

    local catch_fn = block[2] or block['catch']
    if catch_fn then
        assert(type(catch_fn) == "function", "catch block must be a function")
    end

    local finally_fn = block[3] or block['finally']
    if finally_fn then
        assert(type(finally_fn) == "function", "finally block must be a function")
    end

    local ok, err = pcall(try_fn)
    if not ok then
        if catch_fn then
            catch_fn(err)
        end
    end

    if finally_fn then
        finally_fn()
    end
end

-- Reference: https://segmentfault.com/a/1190000023157856

local PENDING = 0
local FULFILLED = 1
local REJECTED = 2

local function resolvePromise(promise, x, resolve, reject)
    if x == nil then
        return resolve(x)
    end

    -- 如果 promise 和 x 指向同一对象，以 TypeError 为据因拒绝执行 promise
    -- 这是为了防止死循环
    if promise == x then
        return reject("The Promise and its return value are the same object.")
    end

    if type(x) == "table" and x.CLASS_NAME == "Promise" then -- 如果 x 为 Promise
        -- 则使 promise 接受 x 的状态
        -- 也就是继续执行x，如果执行的时候拿到一个y，还要继续解析y
        -- 这个if跟下面判断then然后拿到执行其实重复了，可有可无
        x:and_then(function(y)
            resolvePromise(promise, y, resolve, reject)
        end, reject)
    elseif type(x) == "table" or type(x) == "function" then -- 如果 x 为对象或者函数
        if x.and_then == nil then
            return reject("Promise has no and_then method")
        end

        -- 如果 x.and_then 是函数
        if type(x.and_then) == "function" then
            local called = false
            -- 将 x 作为函数的作用域 this 调用之
            -- 传递两个回调函数作为参数，第一个参数叫做 resolvePromise ，第二个参数叫做 rejectPromise
            try {
                function()
                    x:and_then( -- // 如果 resolvePromise 以值 y 为参数被调用，则运行 [[Resolve]](promise, y)
                    function(y)
                        -- 如果 resolvePromise 和 rejectPromise 均被调用，
                        -- 或者被同一参数调用了多次，则优先采用首次调用并忽略剩下的调用
                        if called then
                            return
                        end
                        called = true
                        resolvePromise(promise, y, resolve, reject)
                    end, function(r)
                        -- 如果 rejectPromise 以拒因 r 为参数被调用，则以拒因 r 拒绝 promise
                        if called then
                            return
                        end
                        called = true
                        reject(r)
                    end)
                end,
                catch = function(err)
                    -- 如果调用 then 方法抛出了异常：
                    -- 如果 resolvePromise 或 rejectPromise 已经被调用，则忽略之
                    if called then
                        return
                    end
                    -- 否则以 err 为拒因拒绝 promise
                    reject(err)
                end
            }
        else -- type(x.and_then) ~= "function"
            -- 以 x 为参数执行 promise
            resolve(x);
        end
    else -- x 既不是对象也不是函数
        -- 以 x 为参数执行 promise
        resolve(x);
    end
end

---@class Promise
local Promise = {
    CLASS_NAME = "Promise"
}
Promise.__index = Promise

function Promise.new(executor)
    local this = setmetatable({}, Promise)

    this._state = PENDING
    this._value = nil
    this._reason = nil
    this._onFulfilledCallbacks = {}
    this._onRejectedCallbacks = {}

    local function resolve(value)
        if this._state == PENDING then
            this._state = FULFILLED
            this._value = value
            -- 执行所有成功回调
            for _, callback in ipairs(this._onFulfilledCallbacks) do
                callback(value)
            end
        end
    end

    local function reject(reason)
        if this._state == PENDING then
            this._state = REJECTED
            this._reason = reason
            -- 如果没有失败回调，则抛出错误
            if #this._onRejectedCallbacks == 0 then
                if async_core then
                    async_core:show_error(reason)
                end
                return
            end
            -- 执行所有失败回调
            for _, callback in ipairs(this._onRejectedCallbacks) do
                callback(reason)
            end
        end
    end

    local ok, err = pcall(executor, resolve, reject)
    if not ok then
        reject(err)
    end

    return this
end

function Promise:and_then(onFulfilled, onRejected)
    -- 如果onFulfilled不是函数，给一个默认函数，返回value
    local realOnFulfilled = onFulfilled
    if type(onFulfilled) ~= "function" then
        realOnFulfilled = function(value)
            return value
        end
    end

    -- 如果onRejected不是函数，给一个默认函数，返回reason
    local realOnRejected = onRejected
    if type(onRejected) ~= "function" then
        realOnRejected = function(reason)
            return reason
        end
    end

    local next_promise = nil

    if self._state == FULFILLED then
        next_promise = Promise.new(function(resolve, reject)
            try {
                function()
                    -- 如果 onFulfilled 不是函数且 promise1 成功执行， next_promise 必须成功执行并返回相同的值
                    if type(onFulfilled) ~= "function" then
                        resolve(self._value)
                    else
                        local x = realOnFulfilled(self._value)
                        resolvePromise(next_promise, x, resolve, reject)
                    end
                end,
                catch = function(err)
                    reject(err)
                end
            }
        end)
        return next_promise
    elseif self._state == REJECTED then
        next_promise = Promise.new(function(resolve, reject)
            try {
                function()
                    -- 如果 onRejected 不是函数且 promise1 拒绝执行， next_promise 必须拒绝执行并返回相同的拒因
                    if type(onRejected) ~= "function" then
                        reject(self._reason)
                    else
                        -- 如果promise1的onRejected执行成功了，next_promise应该被resolve
                        local x = realOnRejected(self._reason)
                        resolvePromise(next_promise, x, resolve, reject)
                    end
                end,
                catch = function(err)
                    reject(err)
                end
            }
        end)
        return next_promise
    elseif self._state == PENDING then
        next_promise = Promise.new(function(resolve, reject)
            -- 保存回调，包装错误处理
            table.insert(self._onFulfilledCallbacks, function()
                try {
                    function()
                        local x = realOnFulfilled(self._value)
                        resolvePromise(next_promise, x, resolve, reject)
                    end,
                    catch = function(err)
                        reject(err)
                    end
                }
            end)
            table.insert(self._onRejectedCallbacks, function()
                try {
                    function()
                        local x = realOnRejected(self._reason)
                        resolvePromise(next_promise, x, resolve, reject)
                    end,
                    catch = function(err)
                        reject(err)
                    end
                }
            end)
        end)
        return next_promise
    end

    error("Unreachable: Invalid Promise state")
end

---设置一个捕获错误的回调函数。
---相当于 Promise:and_then(nil, onRejected)
---@param onRejected fun(reason:any) | any
---@return Promise
function Promise:catch(onRejected)
    return self:and_then(nil, onRejected)
end

---设置一个无论状态如何都会执行的回调函数。
---@param onFinally fun()
---@return Promise
function Promise:finally(onFinally)
    return self:and_then(function(value)
        return Promise.resolve(onFinally()):and_then(function()
            return value
        end)
    end, function(reason)
        return Promise.resolve(onFinally()):and_then(function()
            error(reason)
        end)
    end)
end

---将现有对象转为Promise对象。
---如果 param 参数不是 Promise 对象，则返回一个新的 Promise 对象，
---且它的状态为fulfilled。
---@param param any
---@return Promise
function Promise.resolve(param)
    if type(param) == "table" and param.CLASS_NAME == "Promise" then
        -- 如果 param 是一个 Promise
        return param
    end

    -- 创建新的 Promise
    return Promise.new(function(resolve, reject)
        resolve(param)
    end)
end

---返回一个新的Promise实例，该实例的状态为rejected。
---Promise.reject方法的参数reason，会被传递给实例的回调函数。
---@param reason any
---@return Promise
function Promise.reject(reason)
    return Promise.new(function(resolve, reject)
        reject(reason)
    end)
end

---将多个 Promise 实例，包装成一个新的 Promise 实例
---@param promiseList table<number, Promise>
---@return Promise
function Promise.all(promiseList)
    local resPromise = Promise.new(function(resolve, reject)
        local count = 0
        local results = {}
        local length = #promiseList

        if length == 0 then
            return resolve(results)
        end

        for index, promise in ipairs(promiseList) do
            Promise.resolve(promise):and_then(function(value)
                count = count + 1
                results[index] = value
                if count == length then
                    resolve(results)
                end
            end, function(reason)
                reject(reason)
            end)
        end
    end)

    return resPromise
end

---类似 Promise.all，但只返回第一个成功或失败的 Promise 实例的结果。
---@param promiseList table<number, Promise>
---@return Promise
function Promise.race(promiseList)
    local resPromise = Promise.new(function(resolve, reject)
        local length = #promiseList

        if length == 0 then
            return resolve()
        end

        for index, promise in ipairs(promiseList) do
            Promise.resolve(promise):and_then(function(value)
                return resolve(value)
            end, function(reason)
                return reject(reason)
            end)
        end
    end)

    return resPromise
end

function Promise.allSettled()
    error("Promise.allSettled() is not implemented yet.")
end

return Promise
