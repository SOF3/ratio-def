#[macro_export]
macro_rules! units {
    ( #[derive($($derives:path),* $(,)?)]$base:ty:
      $(
          $(#[$meta:meta])*
          $tys:ident;
      )*
      $(
          <$a:path> * <$b:path> = $c:path;
      )*
    ) => {
        $(
            $(#[$meta])*
            #[derive($($derives),*)]
            pub struct $tys(pub $base);

            impl From<$base> for $tys {
                #[inline(always)]
                fn from(base: $base) -> Self {
                    Self(base)
                }
            }

            impl $tys {
                /// Returns the raw value of this struct.
                #[inline(always)]
                pub fn value(self) -> $base {
                    self.0
                }
            }

            impl ::std::ops::Add for $tys {
                type Output = Self;

                #[inline(always)]
                fn add(self, other: Self) -> Self {
                    $tys(self.value() + other.value())
                }
            }

            impl ::std::ops::AddAssign for $tys {
                #[inline(always)]
                fn add_assign(&mut self, other: Self) {
                    self.0 += other.value();
                }
            }

            impl ::std::ops::Sub for $tys {
                type Output = Self;

                #[inline(always)]
                fn sub(self, other: Self) -> Self {
                    $tys(self.value() - other.value())
                }
            }

            impl ::std::ops::SubAssign for $tys {
                #[inline(always)]
                fn sub_assign(&mut self, other: Self) {
                    self.0 -= other.value();
                }
            }

            impl ::std::ops::MulAssign<$base> for $tys {
                #[inline(always)]
                fn mul_assign(&mut self, other: $base) {
                    self.0 *= other;
                }
            }

            impl ::std::ops::DivAssign<$base> for $tys {
                #[inline(always)]
                fn div_assign(&mut self, other: $base) {
                    self.0 /= other;
                }
            }
        )*

        $(
            impl ::std::ops::Mul<$b> for $a {
                type Output = $c;

                #[inline(always)]
                fn mul(self, other: $b) -> $c {
                    $c(self.value() * other.value())
                }
            }

            impl ::std::ops::Mul<$a> for $b {
                type Output = $c;

                #[inline(always)]
                fn mul(self, other: $a) -> $c {
                    $c(self.value() * other.value())
                }
            }

            impl ::std::ops::Div<$b> for $c {
                type Output = $a;

                #[inline(always)]
                fn div(self, other: $b) -> $a {
                    $a(self.value() / other.value())
                }
            }

            impl ::std::ops::Div<$a> for $c {
                type Output = $b;

                #[inline(always)]
                fn div(self, other: $a) -> $b {
                    $b(self.value() / other.value())
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    units! {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)] f64:
        Accel; Veloc; Length; Time; Mass; Force; Energy;

        <Accel> * <Time> = Veloc;
        <Veloc> * <Time> = Length;
        <Mass> * <Accel> = Force;
        <Force> * <Length> = Energy;
    }
}
