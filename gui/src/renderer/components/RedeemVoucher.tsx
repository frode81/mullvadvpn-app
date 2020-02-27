import * as React from 'react';
import { Component, Text, View } from 'reactxp';
import { VoucherResponse } from '../../shared/daemon-rpc-types';
import { messages } from '../../shared/gettext';
import styles from './RedeemVoucherStyles';
import SubmittableTextInput from './SubmittableTextInput';

interface IRedeemVoucherProps {
  submitVoucher: (voucherCode: string) => Promise<VoucherResponse>;
  updateAccountExpiry: (expiry: string) => void;
  onSubmit?: () => void;
  onSuccess?: () => void;
  onFailure?: () => void;
}

interface IRedeemVoucherState {
  inputValue: string;
  response?: VoucherResponse;
}

export default class RedeemVoucher extends Component<IRedeemVoucherProps, IRedeemVoucherState> {
  state = {
    inputValue: '',
    response: undefined,
  };

  public render() {
    return (
      <View>
        <SubmittableTextInput
          value={this.state.inputValue}
          placeholder={'XXXX-XXXX-XXXX-XXXX'}
          onChange={this.onInputChange}
          onSubmit={this.onSubmit}
          buttonDisabled={!this.isValidCode()}
        />
        <RedeemVoucherResponse response={this.state.response} />
      </View>
    );
  }

  private onInputChange = (value: string) => {
    this.setState({ inputValue: value.toUpperCase() });
  };

  private isValidCode(): boolean {
    return this.state.inputValue.length >= 16;
  }

  private onSubmit = async () => {
    if (!this.isValidCode()) {
      return;
    }

    if (this.props.onSubmit) {
      this.props.onSubmit();
    }

    const response = await this.props.submitVoucher(this.state.inputValue);

    if (response.type === 'success') {
      this.setState({ inputValue: '', response });
      this.props.updateAccountExpiry(response.new_expiry);
      if (this.props.onSuccess) {
        this.props.onSuccess();
      }
    } else {
      this.setState({ response });
      if (this.props.onFailure) {
        this.props.onFailure();
      }
    }
  };
}

interface IRedeemVoucherResponseProps {
  response?: VoucherResponse;
}

class RedeemVoucherResponse extends Component<IRedeemVoucherResponseProps> {
  public render() {
    if (this.props.response) {
      switch (this.props.response.type) {
        case 'success':
          return (
            <Text style={styles.redeemVoucherResponseSuccess}>
              {messages.pgettext('redeem-voucher-view', 'Voucher was successfully redeemed.')}
            </Text>
          );
        case 'invalid':
          return (
            <Text style={styles.redeemVoucherResponseError}>
              {messages.pgettext('redeem-voucher-view', 'Voucher code is invalid.')}
            </Text>
          );
        case 'already_used':
          return (
            <Text style={styles.redeemVoucherResponseError}>
              {messages.pgettext('redeem-voucher-view', 'Voucher code has already been used.')}
            </Text>
          );
        case 'error':
          return (
            <Text style={styles.redeemVoucherResponseError}>
              {messages.pgettext('redeem-voucher-view', 'An error occured.')}
            </Text>
          );
      }
    }

    return <View style={styles.redeemVoucherResponseEmpty} />;
  }
}
