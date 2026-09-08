"use client";

import {
  useToolCallElapsed,
  type ToolCallMessagePartComponent,
  type ToolCallMessagePartStatus,
} from "@assistant-ui/react";
import { AlertCircleIcon, CheckIcon, LoaderIcon, XCircleIcon, type LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

type ToolStatus = ToolCallMessagePartStatus["type"];

type ToolActivityLabels = {
  running: string;
  complete: string;
};

const TOOL_ACTIVITY_LABELS: Readonly<Record<string, ToolActivityLabels>> = {
  list_faculty: {
    running: "Looking up faculty",
    complete: "Looked up faculty",
  },
  load_faculty_workspace: {
    running: "Reviewing your courses and preferences",
    complete: "Reviewed your courses and preferences",
  },
  get_scheduling_reference: {
    running: "Checking scheduling details",
    complete: "Checked scheduling details",
  },
  save_faculty_submission: {
    running: "Updating your working draft",
    complete: "Updated your working draft",
  },
};

const STATUS_ICONS: Record<ToolStatus, LucideIcon> = {
  running: LoaderIcon,
  complete: CheckIcon,
  incomplete: XCircleIcon,
  "requires-action": AlertCircleIcon,
};

const formatToolDuration = (milliseconds: number) => {
  if (milliseconds < 1000) return "<1s";
  const seconds = milliseconds / 1000;
  if (seconds < 10) return `${(Math.floor(seconds * 10) / 10).toFixed(1)}s`;
  if (seconds < 60) return `${Math.floor(seconds)}s`;
  return `${Math.floor(seconds / 60)}m ${Math.floor(seconds % 60)}s`;
};

const activityLabel = (toolName: string, status: ToolCallMessagePartStatus | undefined): string => {
  const labels = TOOL_ACTIVITY_LABELS[toolName];
  const runningLabel = labels?.running ?? `Using ${toolName}`;
  switch (status?.type) {
    case "running":
      return runningLabel;
    case "complete":
    case undefined:
      return labels?.complete ?? `Used ${toolName}`;
    case "requires-action":
      return `Waiting to continue ${runningLabel.toLocaleLowerCase()}`;
    case "incomplete":
      if (status.reason === "cancelled") {
        return `Cancelled ${runningLabel.toLocaleLowerCase()}`;
      }
      return `Could not complete ${runningLabel.toLocaleLowerCase()}`;
  }
};

const ToolFallback: ToolCallMessagePartComponent = ({ toolName, status, isError }) => {
  const statusType = status?.type ?? "complete";
  const isRunning = statusType === "running";
  const elapsedMilliseconds = useToolCallElapsed();
  const displayedStatus: ToolStatus = isError && !isRunning ? "incomplete" : statusType;
  const Icon = STATUS_ICONS[displayedStatus];
  const label = isError
    ? `Could not complete ${TOOL_ACTIVITY_LABELS[toolName]?.running.toLocaleLowerCase() ?? `using ${toolName}`}`
    : activityLabel(toolName, status);

  return (
    <div
      data-slot="tool-activity"
      role="status"
      aria-live="polite"
      className="text-muted-foreground flex w-fit items-center gap-2 py-1.5 text-sm"
    >
      <Icon
        aria-hidden="true"
        className={cn("size-4 shrink-0", isRunning && "animate-spin [animation-duration:0.6s]")}
      />
      <span>{label}</span>
      {elapsedMilliseconds !== undefined && (
        <span className="text-xs tabular-nums">{formatToolDuration(elapsedMilliseconds)}</span>
      )}
    </div>
  );
};

export { ToolFallback };
