const Router = require('./router')

/**
 * Example of how router can be used in an application
 *  */
addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

function handler(request) {
  const init = {
    headers: { 'content-type': 'application/json' },
  }
  const body = JSON.stringify({ some: 'json' })
  return new Response(body, init)
}

async function handleRequest(request) {
  const r = new Router()
  const t = new Template({head:"<title>Unite the Workers!</title>"} )
  r.get('/', () => home())
  r.get('/foo', () => foo())
  return await r.route(request)
}

class Template {
  constructor(data) {
    self.head = `<head>${data.head}</head>`
    self.body = `<body>${data.body}</body>`
    self.footer = `<footer>${data.footer}</footer>`
  }

  getHtml(content) {
    return new Response(`
      <!doctype html>
      <html>
      ${self.head}
      ${self.body}
      ${self.footer}
      </html>`, 
      {headers:{"Content-Type":"text/html;charset=utf-8"}})
  }
} 

function html(data={head:"<title>Unite the Workers!</title>",body:"",footer:""}) {
  return new Response(`<!doctype html><html><head>${data.head}</head><body>${data.body}</body><footer>${data.footer}</footer></html>`, {headers:{"Content-Type":"text/html;charset=utf-8"}})
}

function home() {
  return html({body:"<h1>HEEEYY!!! THIZS IS MA WEBPAGE!!</h1><p><a href=\"/foo\">foo</a> <a href=\"/404\">404</a></p>"})
}

function foo() {
  return html({body:"<h1>FFFOOOOOOOO</h1><p><a href=\"/\">Take me home country road</a></p>"})
}
