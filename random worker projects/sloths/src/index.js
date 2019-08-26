const TENOR_API_KEY = "05JQEQC9PO67";
const TOO_MANY_SLOTHS = 50;
const THEY_ARE_ASKING_FOR_A_SLOTH = /^\/(?:[0-9]|[1-4][0-9])$/;
const HTML_HEADERS = new Headers({
  "Content-Type": "text/html; charset=utf-8"
});
const THE_THRONE_OF_THE_CHOSEN = `<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta charset="utf-8"/>
<meta http-equiv="X-UA-Compatible" content="IE=edge"/>
<meta name="description" content="Avery Harnish's personal site"/>
<meta name="author" content="Avery Harnish"/>
<title>Sloth.</title>
<link rel="icon" href="//www.freefavicon.com/freefavicons/animal/sloth-152-301220.png" sizes="32x32">
<link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.1/css/bootstrap.min.css"/>
<style>
* {
    box-sizing: border-box;
}


body {
    padding-top: 50px;
    margin: 0;
    font-family: Arial;
    font-size: 17px;
}
starter-template {
  padding: 40px 15px;
  text-align: center;
}

#myVideo {
    position: fixed;
    left: 0;
    top: 0;
    max-width:100%;
    max-height:100%;
    {{heightOrWidth}}
}

.content {
    position: fixed;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    color: #f1f1f1;
    width: 100%;
    padding: 20px;
}

#myBtn {
    width: 200px;
    font-size: 18px;
    padding: 10px;
    border: none;
    background: #000;
    color: #fff;
    cursor: pointer;
}

#myBtn:hover {
    background: #ddd;
    color: black;
}
</style>
<!-- HTML5 Shim and Respond.js IE8 support of HTML5 elements and media queries -->
<!-- WARNING: Respond.js doesn't work if you view the page via file:// -->
<!--[if lt IE 9]>
<script src="https://oss.maxcdn.com/libs/html5shiv/3.7.0/html5shiv.js"></script>
<script src="https://oss.maxcdn.com/libs/respond.js/1.4.2/respond.min.js"></script>
<![endif]-->
</head>
<body>
<!-- Navigation -->
<nav class="navbar navbar-inverse navbar-fixed-top" role="navigation">
    <div class="container">
        <div class="navbar-header">
            <button type="button" class="navbar-toggle collapsed" data-toggle="collapse" data-target=".navbar-collapse">
                <span class="sr-only">Toggle navigation</span>
                <span class="icon-bar"></span>
                <span class="icon-bar"></span>
                <span class="icon-bar"></span>
            </button>
            <a class="navbar-brand" href="/">Avery Harnish</a>
        </div>
        <div class="collapse navbar-collapse">
            <ul class="nav navbar-nav">
                <li><a href="https://averyharnish.com">Home</a></li>
<li class="active"><a href="https://sloths.averyharnish.com">Sloth</a></li> 
<li><a href="https://megamillions.averyharnish.com">Mega Millions</a></li>                <li><a href="https://github.averyharnish.com">Github</a></li>
                <!-- <li><a href="#about">About</a></li>
                <li><a href="#contact">Contact</a></li> -->
            </ul>
        </div>
        <!--/.nav-collapse -->
    </div>
    <!-- /.container -->
</nav>
<div class="container">
    <div class="row">
        <div class="starter-template">
          <video style="padding-top: 24px;" autoplay muted loop id="myVideo">
          <source src="{{slothUrl}}" type="video/mp4">
          Your browser does not support HTML5 video.
          </video>
    </div>
  </div>
</div>


<script>
var video = document.getElementById("myVideo");
var btn = document.getElementById("myBtn");

function myFunction() {
  if (video.paused) {
    video.play();
    btn.innerHTML = "Pause";
  } else {
    video.pause();
    btn.innerHTML = "Play";
  }
}
</script>
<script src="https://code.jquery.com/jquery-2.1.3.min.js"></script>
<script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.1/js/bootstrap.min.js"></script>
</body>
</html>
`;

const QUERY = "sloth";

addEventListener("fetch", event => {
  event.respondWith(handleEvent(event));
});

async function handleEvent(event) {
  const cache = caches.default;
  const requestUrl = new URL(event.request.url);
  let response = await cache.match(event.request);
  if (!response) {
    if (THEY_ARE_ASKING_FOR_A_SLOTH.test(requestUrl.pathname)) {
      const HTTPSloths = await fetch(
        `https://api.tenor.com/v1/search?q=${QUERY}&key=${TENOR_API_KEY}&limit=${TOO_MANY_SLOTHS}&contentfilter=medium`
      );
      const slothJSON = await HTTPSloths.json();
      const theChosenSloth =
        slothJSON["results"][parseInt(requestUrl.pathname.substring(1), 10)][
          "media"
        ][0]["loopedmp4"];
      const heightOrWidth =
        theChosenSloth.dims[0] < theChosenSloth.dims[1]
          ? "min-width:100%;"
          : "min-height:100%;";
      let slothHtml = THE_THRONE_OF_THE_CHOSEN.replace(
        "{{slothUrl}}",
        theChosenSloth.url
      );
      slothHtml = slothHtml.replace("{{heightOrWidth}}", heightOrWidth);
      response = new Response(slothHtml, { headers: HTML_HEADERS });
    }
    if (response && response.ok) {
      event.waitUntil(cache.put(event.request, response.clone()));
    }
  }

  if (!response) {
    response = await fetch(event.request);
  }

  return response;
}
