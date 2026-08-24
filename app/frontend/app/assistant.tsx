"use client";

import { AssistantRuntimeProvider } from "@assistant-ui/react";
import { useDataStreamRuntime } from "@assistant-ui/react-data-stream";
import { Thread } from "@/components/assistant-ui/thread";

export const Assistant = () => {
  const runtime = useDataStreamRuntime({
    api: "/api/chat",
    protocol: "data-stream",
  });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <div className="grid h-dvh grid-rows-[0.25rem_1fr]">
        <div className="brand-rule" aria-hidden="true" />
        <div className="min-h-0">
          <Thread />
        </div>
      </div>
    </AssistantRuntimeProvider>
  );
};
