import http from "k6/http";
import { check, sleep } from "k6";
import { Rate } from "k6/metrics";

const errorRate = new Rate("errors");

const BASE_URL = __ENV.BASE_URL || "http://localhost:9000";
const INITIAL_RATE = parseInt(__ENV.INITIAL_RATE || "10"); // Initial requests per second
const RATE_INCREMENT = parseInt(__ENV.RATE_INCREMENT || "10"); // Increase in req/s
const STAGE_DURATION = parseInt(__ENV.STAGE_DURATION || "10"); // Duration of each stage in seconds
const NUM_STAGES = parseInt(__ENV.NUM_STAGES || "6"); // Number of stages
const PREALLOC_VUS = parseInt(__ENV.PREALLOC_VUS || "50"); // Preallocated VUs
const MAX_VUS = parseInt(__ENV.MAX_VUS || "200"); // Maximum VUs

function generateStages() {
  const stages = [];
  for (let i = 0; i < NUM_STAGES; i++) {
    const targetRate = INITIAL_RATE + i * RATE_INCREMENT;
    stages.push({
      duration: `${STAGE_DURATION}s`,
      target: targetRate,
    });
  }
  return stages;
}

export const options = {
  scenarios: {
    progressive_load: {
      executor: "ramping-arrival-rate",
      startRate: INITIAL_RATE,
      timeUnit: "1s",
      preAllocatedVUs: PREALLOC_VUS,
      maxVUs: MAX_VUS,
      stages: generateStages(),
    },
  },
  thresholds: {
    http_req_failed: ["rate<0.05"], // HTTP errors should be less than 5%
    http_req_duration: ["p(95)<500"], // 95% of requests should be below 500ms
    errors: ["rate<0.05"], // Error rate should be less than 5%
  },
};

function generateInterestData() {
  const timestamp = Date.now();
  const randomId = Math.floor(Math.random() * 10000);

  return {
    name: `Test User ${randomId}`,
    email: `testuser${randomId}_${timestamp}@example.com`,
  };
}

export function setup() {
  const stages = generateStages();
  const maxRate = INITIAL_RATE + (NUM_STAGES - 1) * RATE_INCREMENT;

  console.log("\n=== Load Test Configuration ===");
  console.log(`Base URL: ${BASE_URL}`);
  console.log(`Initial Rate: ${INITIAL_RATE} req/s`);
  console.log(`Rate Increment: ${RATE_INCREMENT} req/s`);
  console.log(`Stage Duration: ${STAGE_DURATION}s`);
  console.log(`Number of Stages: ${NUM_STAGES}`);
  console.log(`Max Rate: ${maxRate} req/s`);
  console.log(`Preallocated VUs: ${PREALLOC_VUS}`);
  console.log(`Max VUs: ${MAX_VUS}`);
  console.log("\n=== Stages ===");
  stages.forEach((stage, index) => {
    const rate = INITIAL_RATE + index * RATE_INCREMENT;
    console.log(`Stage ${index + 1}: ${rate} req/s for ${stage.duration}`);
  });
  console.log("\n");
}

export default function () {
  const url = `${BASE_URL}/interests`;
  const payload = JSON.stringify(generateInterestData());

  const params = {
    headers: {
      "Content-Type": "application/json",
    },
    tags: { name: "CreateInterest" },
  };

  const response = http.post(url, payload, params);

  const success = check(response, {
    "status is 201": (r) => r.status === 201,
    "response time < 500ms": (r) => r.timings.duration < 500,
  });
  errorRate.add(!success);

  // sleep(0.1);
}
