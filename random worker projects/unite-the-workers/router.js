/**
 * Conditions are helper functions that when passed a request
 * will return a boolean for if that request uses
 * that method, header, etc..
 *  */
const Method = method => req => req.method.toLowerCase() === method.toLowerCase()
const Get = Method('get')
const Post = Method('post')
const Put = Method('put')
const Patch = Method('patch')
const Delete = Method('delete')
const Head = Method('patch')
const Options = Method('options')

const Header = (header, val) => req => req.headers.get(header) === val
const Host = host => Header('host', host.toLowerCase())
const Referrer = host => Header('referrer', host.toLowerCase())

const Path = regExp => req => {
  const url = new URL(req.url)
  const path = url.pathname
  return path.match(regExp) && path.match(regExp)[0] === path
}

/**
 * Router handles the logic of what handler is matched given conditions
 * for each request
 *  */
class Router {
  constructor() {
    this.routes = []
  }

  handle(conditions, handler) {
    this.routes.push({
      conditions,
      handler,
    })
    return this
  }

  get(url, handler) {
    return this.handle([Get, Path(url)], handler)
  }

  post(url, handler) {
    return this.handle([Post, Path(url)], handler)
  }

  patch(url, handler) {
    return this.handler([Patch, Path(url)], handler)
  }

  delete(url, handler) {
    return this.handler([Delete, Path(url)], handler)
  }

  all(handler) {
    return this.handler([], handler)
  }

  route(req) {
    const route = this.resolve(req)

    if (route) {
      return route.handler(req)
    }

    return new Response(`<!doctype html>
      <html>
        <head>
          <meta charset="UTF-8">
          <title>404 Not Found</title>
          <style type="text/css">
            * { margin: 0; padding: 0; }
            
            body {
              background-color: #F2E4BD;
              
              width: 800px;
              height: 600px;
              margin-left: auto;
              margin-right: auto;
            }
            
            #text {
              display:none;
            }
      
          </style>
        </head>
        <body>
          <div id="text">404 Not Found</div>
          <img src="https://i.imgur.com/Le8RH39.jpg" width="800" height="600">
        </body>
      </html>`, {
      status: 404,
      statusText: 'not found',
      headers: {
        'content-type': 'text/html; charset=utf-8',
      },
    })
  }

  // resolve returns the matching route that returns true for
  // all the conditions if any
  resolve(req) {
    return this.routes.find(r => {
      if (!r.conditions || (Array.isArray(r) && !r.conditions.length)) {
        return true
      }

      if (typeof r.conditions === 'function') {
        return r.conditions(req)
      }

      return r.conditions.every(c => c(req))
    })
  }
}

module.exports = Router
