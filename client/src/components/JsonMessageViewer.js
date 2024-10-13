import React from "react";
import JsonView from "@uiw/react-json-view";
import { Container } from "@mui/material";

export default function JsonMessageViewer({ messages }) {
  /**
   * Renders a JSON view inside a container.
   *
   * This function takes a JSON object as input and returns a JSX element that displays
   * the JSON object using the `JsonView` component inside a `Container` component.
   *
   * @param {Object} values - The JSON object to be displayed.
   * @returns {JSX.Element} A JSX element containing the JSON view.
   */
  const renderJson = (values) => {
    return (
      <Container>
        <JsonView value={values} />
      </Container>
    );
  };

  return <>{messages && renderJson(messages)}</>;
}
