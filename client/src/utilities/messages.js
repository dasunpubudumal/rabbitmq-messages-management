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

/**
 * Decodes a Base64 encoded string with UTF-8 encoding.
 *
 * This function takes a Base64 encoded string as input, decodes it, and returns the decoded string
 * with UTF-8 encoding. It uses the `atob` function to perform the Base64 decoding and `TextDecoder`
 * to handle the UTF-8 conversion.
 *
 * @param {string} message - The Base64 encoded string to be decoded.
 * @returns {string} The decoded string with UTF-8 encoding.
 */
const base64Decode = (message) => {
  console.log(`Message: ${message}`);
  const decodedMessage = atob(message);
  const decoder = new TextDecoder("utf-8");
  const decodedArray = Uint8Array.from(decodedMessage, (char) =>
    char.charCodeAt(0),
  );
  return decoder.decode(decodedArray);
};

/**
 * Encodes a string to Base64.
 *
 * This function takes a string as input and returns the Base64 encoded version of the string.
 * It uses the `btoa` function to perform the encoding.
 *
 * @param {string} message - The string to be encoded to Base64.
 * @returns {string} The Base64 encoded string.
 */
const base64Encode = (message) => {
  return btoa(message);
};

/**
 * Truncates a string to a specified length and adds ellipsis if it exceeds that length.
 *
 * @param {string} message - The message to be truncated.
 * @param {number} maxLength - The maximum length of the truncated message.
 * @returns {string} The truncated message.
 */
const truncateMessage = (message, maxLength) => {
  if (message.length > maxLength) {
    return message.substring(0, maxLength) + "...";
  }
  return message;
};

/**
 * Downloads the provided messages as a JSON file.
 * 
 * This function creates a JSON file from the provided messages and triggers a download
 * in the user's browser.
 * 
 * @param {Array|Object} messages - The messages to be downloaded. This can be an array or an object.
 */
const downloadMessages = (messages) => {
  const jsonString = `data:text/json;chatset=utf-8,${encodeURIComponent(
      JSON.stringify(messages)
    )}`;
  const link = document.createElement("a");
  link.href = jsonString;
  link.download = "messages.json";
  link.click();
}

export { filterMessages, base64Decode, base64Encode, truncateMessage, downloadMessages };
