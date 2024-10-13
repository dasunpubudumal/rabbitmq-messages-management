import React from "react";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";

/**
 * Queues component that displays a list of queues.
 *
 * This component receives a list of queues as a prop and renders them.
 *
 * @param {Object} props - The properties passed to the component.
 * @param {Array} props.queues - The list of queues to be displayed.
 * @returns {JSX.Element} A JSX element displaying the list of queues.
 */
export default function Queues({ queues }) {
  return (
    <div>
      <ButtonGroup orientation="vertical" aria-label="Vertical button group">
        {queues.map((queue, index) => (
          <Button key={index}>{queue.name}</Button>
        ))}
      </ButtonGroup>
    </div>
  );
}
