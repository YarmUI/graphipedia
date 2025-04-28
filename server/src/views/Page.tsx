export function Page({ jsPath, cssPath, title, description, image, url }: { jsPath: string; cssPath?: string; title?: string; description?: string; image?: string | null; url?: string }) { 
  return (
    <html lang="ja">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="icon" type="image/x-icon" href="/favicon.ico" />
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-icon.png" />
        <title>{ title }</title>
        <script type="module" src={`/${jsPath}`}></script>
        <link rel="stylesheet" href={`/${cssPath}`} />
        <meta property="og:title" content={title} />
        <meta property="og:description" content={description} />
        { image && <meta property="og:image" content={image} />}
        <meta property="og:type" content="website" />
        <meta property="og:url" content={url} />
      </head>
      <body>
        <div id="root"></div>
      </body>
    </html>
  );
}