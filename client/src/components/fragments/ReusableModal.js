import Modal from "@mui/material/Modal";
import Box from "@mui/material/Box";
import Typography from "@mui/material/Typography";

/**
 * ReusableModal component renders a modal with a title and message.
 *
 * @param {Object} props - The props object.
 * @param {boolean} props.open - Boolean indicating whether the modal is open.
 * @param {function} props.handleClose - Function to handle closing the modal.
 * @param {Object} props.style - Style object for the modal's Box component.
 * @param {string} props.title - The title to be displayed in the modal.
 * @param {string} props.message - The message to be displayed in the modal.
 * @returns {JSX.Element} The rendered ReusableModal component.
 */
export default function ReusableModal({
  open,
  handleClose,
  style,
  title,
  message,
}) {
  return (
    <>
      <Modal
        open={open}
        onClose={handleClose}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description"
      >
        <Box sx={style}>
          <Typography id="modal-modal-title" variant="h6" component="h2">
            {title}
          </Typography>
          <Typography id="modal-modal-description" sx={{ mt: 2 }}>
            {message}
          </Typography>
        </Box>
      </Modal>
    </>
  );
}
