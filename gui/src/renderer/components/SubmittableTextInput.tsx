import * as React from 'react';
import { Animated, Component, Styles, TextInput, Types, View } from 'reactxp';
import { colors } from '../../config.json';
import ImageView from './ImageView';
import styles from './SubmittableTextInputStyles';

interface ISubmittableTextInputProps {
  value: string;
  placeholder?: string;
  onSubmit?: (value: string) => Promise<void> | void;
  onChange?: (value: string) => void;
  buttonDisabled?: boolean;
}

interface ISubmittableTextInputState {
  waiting: boolean;
}

export default class SubmittableTextInput extends Component<
  ISubmittableTextInputProps,
  ISubmittableTextInputState
> {
  state = {
    waiting: false,
  };

  private buttonAnimatedValue = Animated.createValue(this.props.buttonDisabled ? 0.0 : 1.0);
  private buttonAnimation?: Types.Animated.CompositeAnimation;
  private buttonAnimationStyle = Styles.createAnimatedViewStyle({
    backgroundColor: Animated.interpolate(
      this.buttonAnimatedValue,
      [0.0, 1.0, 2.0],
      [colors.white, colors.green, colors.green40],
    ),
  });

  public componentDidUpdate(prevProps: ISubmittableTextInputProps) {
    if (prevProps.buttonDisabled !== this.props.buttonDisabled) {
      this.setButtonActive(this.props.buttonDisabled ? 0.0 : 1.0);
    }
  }

  public render() {
    return (
      <View style={styles.textInputRow}>
        <TextInput
          style={styles.textInput}
          value={this.props.value}
          placeholder={this.props.placeholder}
          placeholderTextColor={colors.blue40}
          autoCorrect={false}
          onChangeText={this.onChange}
          onSubmitEditing={this.onSubmit}
        />
        <Animated.View
          style={[styles.submitButton, this.buttonAnimationStyle]}
          onPress={this.onSubmit}>
          <ImageView source="icon-arrow" height={16} width={24} tintColor="rgb(255, 255, 255)" />
        </Animated.View>
      </View>
    );
  }

  private onChange = (value: string) => {
    if (this.props.onChange) {
      this.props.onChange(value);
    }
  };

  private onSubmit = async () => {
    if (!this.props.buttonDisabled && !this.state.waiting && this.props.onSubmit) {
      this.setState({ waiting: true });
      this.setButtonActive(2.0);
      await this.props.onSubmit(this.props.value);
      this.setState({ waiting: false });
      this.setButtonActive(this.props.buttonDisabled ? 0.0 : 1.0);
    }
  };

  private setButtonActive(toValue: number) {
    const animation = Animated.timing(this.buttonAnimatedValue, {
      toValue,
      easing: Animated.Easing.Linear(),
      duration: 250,
    });

    const oldAnimation = this.buttonAnimation;
    if (oldAnimation) {
      oldAnimation.stop();
    }

    animation.start();

    this.buttonAnimation = animation;
  }
}
