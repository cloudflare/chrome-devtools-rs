const amphtmlValidator = require("amphtml-validator");

/* global addEventListener */
addEventListener("fetch", event => {
  event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  const parsedUrl = new URL(request.url);
  if (parsedUrl.pathname === "/validate") {
    const validateUrl = parsedUrl.searchParams.get("url");
    const candidateResponse = await fetch(validateUrl);
    const candidateText = await candidateResponse.text();
    amphtmlValidator.getInstance().then(function(validator) {
      var result = validator.validateString(candidateBody);
      (result.status === "PASS" ? console.log : console.error)(result.status);
      for (var ii = 0; ii < result.errors.length; ii++) {
        var error = result.errors[ii];
        var msg =
          "line " + error.line + ", col " + error.col + ": " + error.message;
        if (error.specUrl !== null) {
          msg += " (see " + error.specUrl + ")";
        }
        (error.severity === "ERROR" ? console.error : console.warn)(msg);
      }
    });
  }
  return await fetch(request);
}
