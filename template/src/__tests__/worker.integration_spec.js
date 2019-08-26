import { createTestApp } from "cloudflare-worker-local";
import supertest from "supertest";
import express from "express";
import fs from "fs";

const workerContent = fs.readFileSync(`${__dirname}/../../dist/main.js`);

describe("My New Worker", () => {
  it("Adds a header on 200 responses", async () => {
    const upstreamApp = express();
    upstreamApp.get("/route", (req, res) => res.end("Success"));
    await supertest(createTestApp(workerContent, upstreamApp))
      .get("/route")
      .expect("Foo", "Bar")
      .expect(200, "Success");
  });

  it("Adds a different header on 404", async () => {
    // all routes are 404
    const upstreamApp = express();

    await supertest(createTestApp(workerContent, upstreamApp))
      .get("/route")
      .expect("Foo", "Not Bar")
      .expect(404);
  });
});
