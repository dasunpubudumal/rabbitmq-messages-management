import React from "react";
import JsonView from "@uiw/react-json-view";
import { Container, IconButton } from "@mui/material";
import { Typography } from "@mui/material";
import { Box } from "@mui/material";
import FileDownloadIcon from "@mui/icons-material/FileDownload";
import { Tooltip } from "@mui/material";
import { downloadMessages } from "../../utilities/messages";

/**
 * JsonMessageViewer component renders a list of JSON messages using the renderJson function.
 *
 * @param {Object} props - The props object.
 * @param {Array} props.messages - Array of JSON messages to be displayed.
 * @returns {JSX.Element} The rendered JsonMessageViewer component.
 */
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
        {messages && messages.length === 0 && (
          <Typography sx={{ color: "text.secondary" }}>No messages</Typography>
        )}
        {messages && messages.length > 0 && (
          <>
            <JsonView
              value={values}
              collapsed={true}
              displayDataTypes={true}
              displayObjectSize={true}
            />
            <Box display="flex" justifyContent="flex-end" sx={{ mt: 0 }}>
              <Tooltip title="Download">
                <IconButton
                  size="large"
                  variant="contained"
                  onClick={() => downloadMessages(messages)}
                >
                  <FileDownloadIcon color="warning" />
                </IconButton>
              </Tooltip>
            </Box>
          </>
        )}
      </Container>
    );
  };

  return <>{messages && renderJson(messages)}</>;
}
