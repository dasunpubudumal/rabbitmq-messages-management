/**
 * Filters and categorizes messages based on their payload encoding.
 *
 * This function processes an array of messages and categorizes them into
 * three groups: JSON messages, string messages, and base64 messages.
 * - JSON messages are those with a payload encoding of "string" and valid JSON.
 * - String messages are those with a payload encoding of "string" but invalid JSON.
 * - Base64 messages are those with a payload encoding of "base64".
 *
 * @param {Array} messages - The array of messages to be filtered and categorized.
 * @returns {Object} An object containing three arrays:
 *   - jsonMessages: An array of messages that are valid JSON.
 *   - stringMessages: An array of messages that are strings but not valid JSON.
 *   - base64Messages: An array of messages with a payload encoding of "base64".
 */
const filterMessages = (messages) => {
  let stringMessages = [];
  let jsonMessages = messages
    .filter((message) => message.payload_encoding === "string")
    .filter((message) => {
      try {
        JSON.parse(message.payload);
        return true;
      } catch (e) {
        stringMessages.push(message.payload);
        return false;
      }
    })
    .map((message) => JSON.parse(message.payload));
  return {
    jsonMessages,
    stringMessages,
    base64Messages: messages.filter(
      (message) => message.payload_encoding === "base64",
    ),
  };
};

export { filterMessages };
