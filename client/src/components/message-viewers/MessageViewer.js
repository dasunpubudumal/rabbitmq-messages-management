import React from "react";
import JsonMessageViewer from "./JsonMessageViewer";
import ReusableAccordion from "../fragments/ReusableAccordion";
import Base64EncodedMessageViewer from "./Base64EncodedMessageViewer";
import StringMessageViewer from "./StringMessageViewer";

export default function MessageViewer({
  jsonMessages,
  base64EncodedMessages,
  stringMessages,
}) {
  return (
    <>
      <ReusableAccordion
        title="JSON Messages"
        ChildComponent={JsonMessageViewer}
        messages={jsonMessages}
      />
      <ReusableAccordion
        title="Base64-encoded Messages"
        ChildComponent={Base64EncodedMessageViewer}
        messages={base64EncodedMessages}
      />
      <ReusableAccordion
        title="String Messages"
        ChildComponent={StringMessageViewer}
        messages={stringMessages}
      />
    </>
  );
}
