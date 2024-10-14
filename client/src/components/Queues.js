import React from "react";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import { Box, Typography } from "@mui/material";

/**
 * Queues component that displays a list of queues.
 *
 * This component receives a list of queues as a prop and renders them.
 *
 * @param {Object} props - The properties passed to the component.
 * @param {Array} props.queues - The list of queues to be displayed.
 * @returns {JSX.Element} A JSX element displaying the list of queues.
 */
export default function Queues({ queues, handleSelectedQueueChange }) {
  return (
    <>
      <Box sx={{ bt: 4 }}>
        {queues.length > 0 && (
          <>
            <Typography variant="h5" gutterBottom>
              Queues
            </Typography>
            <ButtonGroup
              orientation="vertical"
              aria-label="Vertical button group"
            >
              {queues.map((queue, index) => (
                <Button
                  key={index}
                  onClick={() => handleSelectedQueueChange(queue.name)}
                  color="warning"
                >
                  {queue.name}
                </Button>
              ))}
            </ButtonGroup>
          </>
        )}
        {queues.length === 0 && (
          <Typography variant="subtitle1" gutterBottom>
            Please select a vhost.
          </Typography>
        )}
      </Box>
    </>
  );
}
