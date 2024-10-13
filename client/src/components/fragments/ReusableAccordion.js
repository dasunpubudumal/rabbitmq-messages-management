import Accordion from "@mui/material/Accordion";
import AccordionSummary from "@mui/material/AccordionSummary";
import AccordionDetails from "@mui/material/AccordionDetails";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import Typography from "@mui/material/Typography";

/**
 * Reusable Accordion component that accepts a child component to render inside AccordionDetails.
 *
 * @param {string} title - The title of the accordion.
 * @param {number} messageCount - The number of messages to display in the summary.
 * @param {React.Component} ChildComponent - The child component to render inside AccordionDetails.
 * @param {Array} messages - The messages to be passed to the child component.
 * @returns {JSX.Element} The rendered accordion component.
 */
export default function ReusableAccordion({ title, ChildComponent, messages }) {
  return (
    <Accordion>
      <AccordionSummary
        expandIcon={<ExpandMoreIcon />}
        aria-controls="panel1-content"
        id="panel1-header"
      >
        <Typography sx={{ width: "33%", flexShrink: 0 }}>
          {title}
        </Typography>
        <Typography sx={{ color: "text.secondary" }}>
          # of messages: {messages && messages.length} {!messages && 0}
        </Typography>
      </AccordionSummary>
      <AccordionDetails>
        <ChildComponent messages={messages} />
      </AccordionDetails>
    </Accordion>
  );
};
