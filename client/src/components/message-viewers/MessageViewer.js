import React from "react";
import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import Typography from "@mui/material/Typography";
import JsonMessageViewer from "./JsonMessageViewer";

export default function MessageViewer({
  jsonMessages,
  base64EncodedMessages,
  stringMessages,
}) {
  return (
    <>
      <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="panel1-content"
          id="panel1-header"
        >
          <Typography sx={{ width: "33%", flexShrink: 0 }}>
            Json Messages
          </Typography>
          <Typography sx={{ color: "text.secondary" }}># of messages: {jsonMessages.length}</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <JsonMessageViewer messages={jsonMessages} />
        </AccordionDetails>
      </Accordion>
    </>
  );
}
