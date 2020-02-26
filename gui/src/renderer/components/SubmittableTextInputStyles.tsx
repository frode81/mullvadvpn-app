import { Styles } from 'reactxp';
import { colors } from '../../config.json';

export default {
  textInputRow: Styles.createViewStyle({
    borderRadius: 4,
    flexDirection: 'row',
  }),
  textInput: Styles.createTextInputStyle({
    flex: 1,
    overflow: 'hidden',
    paddingTop: 14,
    paddingLeft: 14,
    paddingRight: 14,
    paddingBottom: 14,
    fontFamily: 'Open Sans',
    fontSize: 13,
    fontWeight: '600',
    lineHeight: 26,
    color: colors.blue,
    backgroundColor: colors.white,
  }),
  submitButton: Styles.createViewStyle({
    paddingHorizontal: 12,
    justifyContent: 'center',
  }),
};
