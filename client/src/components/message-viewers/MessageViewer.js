import React from "react";
import JsonMessageViewer from "./JsonMessageViewer";
import ReusableAccordion from "../fragments/ReusableAccordion";
import Base64EncodedMessageViewer from "./Base64EncodedMessageViewer";
import StringMessageViewer from "./StringMessageViewer";

/**
 * MessageViewer component renders multiple ReusableAccordion components,
 * each displaying a different type of message.
 *
 * @param {Object} props - The props object.
 * @param {Array} props.jsonMessages - Array of JSON encoded messages.
 * @param {Array} props.base64EncodedMessages - Array of Base64 encoded messages.
 * @param {Array} props.stringMessages - Array of string messages.
 * @returns {JSX.Element} The rendered MessageViewer component.
 */
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
