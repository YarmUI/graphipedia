import { serve } from '@hono/node-server'
import { Hono } from 'hono'
import { serveStatic } from '@hono/node-server/serve-static'
import { Page } from './views/Page'
import * as fs from 'fs'

const app = new Hono()

const manifest_path = '../frontend/dist/.vite/manifest.json' //TODO ENVを読む
const manifestStr = fs.readFileSync(manifest_path, 'utf-8')
const manifest = JSON.parse(manifestStr)
const entry = manifest['index.html']
const jsPath = entry.file
const cssPath = entry.css?.[0]

const api_endpoint = 'http://localhost:3000' // TODO ENVを読む

app.all('/api/*', async (c) => {
  const url = new URL(c.req.url)
  const apiUrl = api_endpoint + url.pathname + url.search
  const req = new Request(apiUrl, {
    method: c.req.method,
    headers: c.req.raw.headers,
    body: ['GET', 'HEAD'].includes(c.req.method) ? undefined : c.req.raw.body,
  })
  const res = await fetch(req)
  return new Response(res.body, {
    status: res.status,
    headers: res.headers,
  })
})

app.use('/*', async (c, next) => {
  if (c.req.path === '/') {
    return await next();
  }
  return await serveStatic({ root: '../frontend/dist' })(c, next)
})

app.get('/', async (c) => {
  const start = c.req.query('start')
  const end = c.req.query('end')
  let title = 'Graphipedia'
  let description = 'スタートの記事からゴールの記事までのWikipediaのリンクを探索します。';
  const search = c.req.url.includes('?') ? c.req.url.substring(c.req.url.indexOf('?')) : ''
  const url = `https://graphipedia.dog-right.dev/${search}`
  let image = null;

  if (start && end) {
    const api_res = await fetch(`${api_endpoint}/api/graph_search?start=${start}&end=${end}`);
    if (api_res.ok) {
      const data = await api_res.json();
      title = description = `${data.start_node.title}から${data.end_node.title}へは${data.end_node.distance}リンクで到達できます。`;
      title += " - Graphipedia"
      image = `https://graphipedia.dog-right.dev/snapshot${search}`
    }
  }

  return c.html(<Page jsPath={jsPath} cssPath={cssPath} title={title} image={image} url={url}/>)
})

serve({
  fetch: app.fetch,
  port: 3001
}, (info) => {
  console.log(`Server is running on http://localhost:${info.port}`)
})
