import * as React from "react";
import Alert from "@mui/material/Alert";
import AlertTitle from "@mui/material/AlertTitle";

/**
 * ReusableAlertComponent
 *
 * This component renders an alert with a title and description.
 *
 * @param {Object} props - The props for the component.
 * @param {string} props.severity - The severity level of the alert. Can be "error", "warning", "info", or "success".
 * @param {string} props.title - The title of the alert.
 * @param {string} props.description - The description of the alert.
 *
 * @returns {JSX.Element} The rendered alert component.
 */
export default function ReusableAlertComponent({
  severity,
  title,
  description,
}) {
  return (
    <>
      <Alert severity={severity} sx={{ mb: 4 }}>
        <AlertTitle>{title}</AlertTitle>
        {description}
      </Alert>
    </>
  );
}
