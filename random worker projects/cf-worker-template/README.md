# SQL Gateway Worker Prototype

This worker will log all user agents that access each route the worker is enabled on. These statistics can be viewed by appending `/stat` to the route you want to view statistics for.

For instance, if you wanted to get the user agent statistics for `https://example.org/stat`, you would go to `https://example.org/stat`

## Usage

To compile this worker, clone the repository, run `npm install` followed by `npm run build`, and put the output from the `target` directory in the worker console. This worker is subdomain agnostic, so if you want to enable it across the entire site, set the route to `*example.com/*`

## Intricacies

This worker excludes any route behind a `/static` path, and the `/stat` routes are not logged for obvious reasons.

For now this worker depends entirely on Ahrenn's instance of the database being up and running, so I'd wait until a more permanent solution arises.

The new Cloudflared feature exposes an endpoint that allows an authenticated user to execute arbitrary SQL.

## Existing Demo

I wouldn't put this in front of your site as is, given its instability. You can check to see if it's working at `averyharnish.us`. It may go up and down periodically as I work on other workers and Ahrenn shuts down his laptop.