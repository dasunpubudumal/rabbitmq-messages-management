import React from "react";

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
      <ul>
        {queues.map((queue, index) => (
          <li key={index}>{queue.name}</li>
        ))}
      </ul>
    </div>
  );
}
