function randomBytes(length) {
  var result           = '';
  var characters       = '0123456789abcdef';
  var charactersLength = characters.length;
  // length * 2 because 2chars = 1byte
  for ( var i = 0; i < length * 2; i++ ) {
      result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

export default randomBytes;