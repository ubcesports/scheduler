import express from "express";

import { api } from "./api";
import {
  Schedule,
  Availability,
  GenerateRequest,
  GenerateResponse,
  Parameters,
  Slots,
} from "./types";

const app = express();

app.set("view engine", "pug");
app.set("views", "views");

app.use(express.urlencoded({ extended: true }));
app.use(express.json());

app.get("/", async (req, res) => {
  const schedulesResult = await api<Schedule[]>("/schedules");
  const availabilitiesResult = await api<Availability[]>("/availabilities");
  const parametersResult = await api<Parameters>("/parameters");

  const currentSchedule = parametersResult.success
    ? parametersResult.data?.schedule
    : null;

  const currentAvailability = parametersResult.success
    ? parametersResult.data?.availability
    : null;

  availabilitiesResult?.data?.sort(
    (a, b) =>
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
  );

  res.render("index", {
    schedules: schedulesResult.success ? schedulesResult.data : [],
    availabilities: availabilitiesResult.success
      ? availabilitiesResult.data
      : [],
    currentSchedule,
    currentAvailability,
    error: !schedulesResult.success ? schedulesResult.error : null,
  });
});

app.get("/schedule/:id", async (req, res) => {
  const slotsResult = await api<Slots>(`/slots`);
  const scheduleResult = await api<Schedule>(`/schedule/${req.params.id}`);

  if (!slotsResult.success) {
    return res.render("error", {
      message: "Slots not found",
      error: slotsResult.error,
    });
  }

  if (!scheduleResult.success) {
    return res.render("error", {
      message: "Schedule not found",
      error: scheduleResult.error,
    });
  }

  const slots = slotsResult.data!;
  const { assignments } = scheduleResult.data!;

  res.render("schedule", {
    schedule: scheduleResult.data,
    scheduleId: req.params.id,
    assignments: sortedData(assignments, slots),
  });
});

app.get("/availability/:id", async (req, res) => {
  const slotsResult = await api<Slots>(`/slots`);
  const availabilityResult = await api<Availability>(
    `/availability/${req.params.id}`
  );

  if (!slotsResult.success) {
    return res.render("error", {
      message: "Slots not found",
      error: slotsResult.error,
    });
  }

  if (!availabilityResult.success) {
    return res.render("error", {
      message: "Availability not found",
      error: availabilityResult.error,
    });
  }

  const slots = slotsResult.data!;
  const { entries } = availabilityResult.data!;

  res.render("availability", {
    availability: availabilityResult.data,
    availabilityId: req.params.id,
    entries: sortedData(entries, slots).map((k) => ({
      id: k.id,
      data: k.data.map((l) => l.name),
    })),
  });
});

app.get("/generate", async (req, res) => {
  const parametersResult = await api<Parameters>("/parameters");
  const currentSchedule = parametersResult.success
    ? parametersResult.data?.schedule
    : null;

  const prefill = "parent" in req.query ? req.query.parent : currentSchedule;
  res.render("generate", { prefill });
});

app.post("/schedule/generate", async (req, res) => {
  const parentId = req.body.parent || null;
  const result = await api<GenerateResponse>("/schedule/generate", "POST", {
    parent: parentId,
  });

  if (result.success) {
    res.redirect(`/schedule/${result.data!.id}`);
  } else {
    res.render("error", {
      message: "Failed to generate schedule",
      error: result.error,
    });
  }
});

app.get("/import", async (req, res) => {
  res.render("import");
});

app.post("/availability/import", async (req, res) => {
  const { url } = req.body;
  const result = await api("/availability/import", "POST", {
    format: "w2m",
    source: url,
  });

  if (result.success) {
    res.redirect("/");
  } else {
    res.render("error", {
      message: "Failed to import availability",
      error: result.error,
    });
  }
});

const PORT = process.env.PORT || 3001;
app.listen(PORT, () => {
  console.log(`Web server running on http://localhost:${PORT}`);
});

function sortedData(
  data: Record<string, { id: string; name: string }[]>,
  slots: { id: string; w2m_id: number }[]
): { id: string; data: { id: string; name: string }[] }[] {
  const map: Record<string, number> = {};
  slots.forEach((slot) => (map[slot.id] = slot.w2m_id));

  return Object.entries(data)
    .map((data) => ({ id: data[0], data: data[1] }))
    .sort((a, b) => map[a.id] - map[b.id]);
}
