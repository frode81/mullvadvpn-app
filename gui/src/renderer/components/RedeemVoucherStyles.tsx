import { Styles } from 'reactxp';
import { colors } from '../../config.json';

export default {
  redeemVoucherResponseSuccess: Styles.createTextStyle({
    marginTop: 8,
    fontFamily: 'Open Sans',
    fontSize: 13,
    fontWeight: '600',
    lineHeight: 20,
    color: colors.green,
  }),
  redeemVoucherResponseError: Styles.createTextStyle({
    marginTop: 8,
    fontFamily: 'Open Sans',
    fontSize: 13,
    fontWeight: '800',
    lineHeight: 20,
    color: colors.red,
  }),
  redeemVoucherResponseEmpty: Styles.createViewStyle({
    height: 20,
    marginTop: 8,
  }),
};
